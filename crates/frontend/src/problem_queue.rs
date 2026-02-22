//! Problem queue — pre-fetches batches of problems to reduce API requests.

use std::collections::VecDeque;

use leptos::prelude::*;
use leptos::task::spawn_local;
use locus_common::{
    ProblemResponse,
    constants::{PROBLEM_BATCH_SIZE, PROBLEM_QUEUE_REFILL_THRESHOLD},
};

use crate::api;

/// A reactive problem queue that pre-fetches problems in batches.
///
/// Call `next()` to pop a problem. The queue automatically refills in the
/// background when it gets low.
#[derive(Clone, Copy)]
pub struct ProblemQueue {
    queue: RwSignal<VecDeque<ProblemResponse>>,
    loading: RwSignal<bool>,
    error: RwSignal<Option<String>>,
    practice: bool,
}

impl ProblemQueue {
    pub fn new(practice: bool) -> Self {
        Self {
            queue: RwSignal::new(VecDeque::new()),
            loading: RwSignal::new(false),
            error: RwSignal::new(None),
            practice,
        }
    }

    /// Whether a fetch is currently in progress.
    pub fn loading(&self) -> bool {
        self.loading.get()
    }

    /// Last error from a fetch, if any.
    pub fn error(&self) -> Option<String> {
        self.error.get()
    }

    /// Clear the error.
    pub fn clear_error(&self) {
        self.error.set(None);
    }

    /// Discard all queued problems (e.g. when topic changes).
    pub fn clear(&self) {
        self.queue.write().clear();
    }

    /// Pop the next problem from the queue, triggering a background refill if
    /// the queue is getting low.
    pub fn next(&self, topic: Option<String>, subtopics: Vec<String>) -> Option<ProblemResponse> {
        let problem = self.queue.write().pop_front();

        if self.queue.read().len() <= PROBLEM_QUEUE_REFILL_THRESHOLD && !self.loading.get() {
            self.fetch(topic, subtopics);
        }

        problem
    }

    /// Fetch a batch of problems from the API.
    pub fn fetch(&self, topic: Option<String>, subtopics: Vec<String>) {
        if self.loading.get() {
            return;
        }
        self.loading.set(true);
        self.error.set(None);

        let queue = self.queue;
        let loading = self.loading;
        let error = self.error;
        let practice = self.practice;

        spawn_local(async move {
            let result = api::get_problems(
                practice,
                topic.as_deref(),
                Some(&subtopics),
                PROBLEM_BATCH_SIZE,
            )
            .await;

            match result {
                Ok(problems) => {
                    queue.write().extend(problems);
                }
                Err(e) => {
                    error.set(Some(e.message));
                }
            }
            loading.set(false);
        });
    }
}
