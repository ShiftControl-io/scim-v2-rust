//! Peak-allocation bound for hostile filters.
//!
//! Devin's review of #49 (SEC-1) observed that a term limit checked after
//! parsing bounds nothing about memory: the whole AST exists before the check
//! runs. The limits are now charged while the parser runs, and this test is
//! the direct falsifier — it measures peak live allocation across the parse
//! of inputs far past both limits and asserts it stays small and independent
//! of input length. Its own test binary, because the counting allocator is
//! process-global and the unit suite runs in parallel.
#![cfg(feature = "filter")]

use scim_v2::filter::{Filter, MAX_FILTER_DEPTH, MAX_FILTER_TERMS, PatchPath};
use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicUsize, Ordering::SeqCst};

struct Counting;

static LIVE: AtomicUsize = AtomicUsize::new(0);
static PEAK: AtomicUsize = AtomicUsize::new(0);

unsafe impl GlobalAlloc for Counting {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let p = unsafe { System.alloc(layout) };
        if !p.is_null() {
            let live = LIVE.fetch_add(layout.size(), SeqCst) + layout.size();
            PEAK.fetch_max(live, SeqCst);
        }
        p
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe { System.dealloc(ptr, layout) };
        LIVE.fetch_sub(layout.size(), SeqCst);
    }
}

#[global_allocator]
static ALLOCATOR: Counting = Counting;

/// Peak bytes allocated above the baseline while `f` runs.
fn peak_during(f: impl FnOnce()) -> usize {
    let base = LIVE.load(SeqCst);
    PEAK.store(base, SeqCst);
    f();
    PEAK.load(SeqCst) - base
}

/// A few times the measured peak (≈230 KiB for a chain at the term limit,
/// dominated by the 1024-operand `Vec` and its last doubling), so a
/// regression to post-parse enforcement — peak ≈ 15× input for the term case
/// — fails loudly while allocator noise does not.
const BOUND: usize = 512 * 1024;

/// Two inputs of the same hostile shape at different lengths must peak the
/// same: allocation before rejection depends on the limits, not on the input.
const SAME_PEAK_TOLERANCE: usize = 4 * 1024;

fn or_chain(term: &str, n: usize) -> String {
    std::iter::repeat_n(term, n)
        .collect::<Vec<_>>()
        .join(" or ")
}

fn not_nest(term: &str, n: usize) -> String {
    format!("{}{term}{}", "not (".repeat(n), ")".repeat(n))
}

#[test]
fn hostile_filters_are_rejected_within_a_bounded_allocation() {
    // The shared parsers compile their lexer on first use; keep that constant
    // out of every measurement.
    "title pr".parse::<Filter>().unwrap();
    "emails[type pr].value".parse::<PatchPath>().unwrap();

    type Build = Box<dyn Fn(usize) -> String>;
    type Rejected = Box<dyn Fn(&str) -> bool>;
    let cases: [(&str, Build, Rejected); 4] = [
        (
            "long filter chain",
            Box::new(|k| or_chain("title pr", k * MAX_FILTER_TERMS)),
            Box::new(|s| s.parse::<Filter>().is_err()),
        ),
        (
            "deep filter nest",
            Box::new(|k| not_nest("title pr", k * MAX_FILTER_DEPTH)),
            Box::new(|s| s.parse::<Filter>().is_err()),
        ),
        (
            "long path chain",
            Box::new(|k| {
                format!(
                    "emails[{}].value",
                    or_chain("type pr", k * MAX_FILTER_TERMS)
                )
            }),
            Box::new(|s| s.parse::<PatchPath>().is_err()),
        ),
        (
            "deep path nest",
            Box::new(|k| {
                format!(
                    "emails[{}].value",
                    not_nest("type pr", k * MAX_FILTER_DEPTH)
                )
            }),
            Box::new(|s| s.parse::<PatchPath>().is_err()),
        ),
    ];

    for (name, build, rejected) in &cases {
        let small = build(100);
        let large = build(1_000);
        let peak_small =
            peak_during(|| assert!(rejected(&small), "{name} (100×) must be rejected"));
        let peak_large =
            peak_during(|| assert!(rejected(&large), "{name} (1000×) must be rejected"));
        eprintln!(
            "{name}: {} bytes → peak {peak_small}; {} bytes → peak {peak_large}",
            small.len(),
            large.len()
        );
        assert!(
            peak_small < BOUND,
            "{name}: peak {peak_small} bytes for a {}-byte input",
            small.len()
        );
        assert!(
            peak_large <= peak_small + SAME_PEAK_TOLERANCE,
            "{name}: peak grew with input length, {peak_small} → {peak_large}"
        );
    }

    // The largest *accepted* filter lives in the same envelope, so the bound is
    // a property of the limits rather than of rejection.
    let at_limit = or_chain("title pr", MAX_FILTER_TERMS);
    let peak = peak_during(|| {
        let f: Filter = at_limit.parse().expect("a chain at the term limit parses");
        assert!(matches!(&f, Filter::Or(items) if items.len() == MAX_FILTER_TERMS));
    });
    eprintln!(
        "at limit: input {} bytes, peak {peak} bytes",
        at_limit.len()
    );
    assert!(peak < BOUND, "at limit: peak {peak} bytes");
}
