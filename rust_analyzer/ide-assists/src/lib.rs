//! `assists` crate provides a bunch of code assists, also known as code actions
//! (in LSP) or intentions (in IntelliJ).
//!
//! An assist is a micro-refactoring, which is automatically activated in
//! certain context. For example, if the cursor is over `,`, a "swap `,`" assist
//! becomes available.
//!
//! ## Assists Guidelines
//!
//! Assists are the main mechanism to deliver advanced IDE features to the user,
//! so we should pay extra attention to the UX.
//!
//! The power of assists comes from their context-awareness. The main problem
//! with IDE features is that there are a lot of them, and it's hard to teach
//! the user what's available. Assists solve this problem nicely: 💡 signifies
//! that *something* is possible, and clicking on it reveals a *short* list of
//! actions. Contrast it with Emacs `M-x`, which just spits an infinite list of
//! all the features.
//!
//! Here are some considerations when creating a new assist:
//!
//! * It's good to preserve semantics, and it's good to keep the code compiling,
//!   but it isn't necessary. Example: "flip binary operation" might change
//!   semantics.
//! * Assist shouldn't necessary make the code "better". A lot of assist come in
//!   pairs: "if let <-> match".
//! * Assists should have as narrow scope as possible. Each new assists greatly
//!   improves UX for cases where the user actually invokes it, but it makes UX
//!   worse for every case where the user clicks 💡 to invoke some *other*
//!   assist. So, a rarely useful assist which is always applicable can be a net
//!   negative.
//! * Rarely useful actions are tricky. Sometimes there are features which are
//!   clearly useful to some users, but are just noise most of the time. We
//!   don't have a good solution here, our current approach is to make this
//!   functionality available only if assist is applicable to the whole
//!   selection. Example: `sort_items` sorts items alphabetically. Naively, it
//!   should be available more or less everywhere, which isn't useful. So
//!   instead we only show it if the user *selects* the items they want to sort.
//! * Consider grouping related assists together (see [`Assists::add_group`]).
//! * Make assists robust. If the assist depends on results of type-inference too
//!   much, it might only fire in fully-correct code. This makes assist less
//!   useful and (worse) less predictable. The user should have a clear
//!   intuition when each particular assist is available.
//! * Make small assists, which compose. Example: rather than auto-importing
//!   enums in `add_missing_match_arms`, we use fully-qualified names. There's a
//!   separate assist to shorten a fully-qualified name.
//! * Distinguish between assists and fixits for diagnostics. Internally, fixits
//!   and assists are equivalent. They have the same "show a list + invoke a
//!   single element" workflow, and both use [`Assist`] data structure. The main
//!   difference is in the UX: while 💡 looks only at the cursor position,
//!   diagnostics squigglies and fixits are calculated for the whole file and
//!   are presented to the user eagerly. So, diagnostics should be fixable
//!   errors, while assists can be just suggestions for an alternative way to do
//!   something. If something *could* be a diagnostic, it should be a
//!   diagnostic. Conversely, it might be valuable to turn a diagnostic with a
//!   lot of false errors into an assist.
//!
//! See also this post:
//! <https://rust-analyzer.github.io/blog/2020/09/28/how-to-make-a-light-bulb.html>

pub mod assist_config;
pub mod assist_context;
pub mod utils;

pub use assist_config::AssistConfig;
pub use ide_db::assists::{
    Assist, AssistId, AssistKind, AssistResolveStrategy, GroupLabel, SingleResolve,
};