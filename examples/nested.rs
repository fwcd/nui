#![feature(type_alias_impl_trait)]

use nui::{Text, VStack, View, Bind, Button, State, Context};

struct IncrementView<F> {
    action: F,
}

impl<F> IncrementView<F> {
    pub fn new(action: F) -> Self {
        Self { action }
    }
}

impl<F> Bind for IncrementView<F> {}

impl<F> View for IncrementView<F> where F: Fn() + Clone + 'static {
    type Body = impl View;

    fn body(&self) -> Self::Body {
        let action = self.action.clone();
        Button::new(Text::new("Increment"), move || {
            action()
        })
    }
}

struct CounterView {
    count: State<i32>,
}

impl CounterView {
    fn new() -> Self {
        Self { count: State::new(0) }
    }
}

impl Bind for CounterView {
    fn bind(&mut self, context: &Context) {
        self.count.link(context.storage().clone(), context.id_path().clone(), 0);
    }
}

impl View for CounterView {
    type Body = impl View;

    fn body(&self) -> Self::Body {
        let count = self.count.clone();
        VStack::new((
            Text::new(format!("Count: {}", count.get())),
            IncrementView::new(move || {
                count.set(count.get() + 1);
            })
        ))
    }
}

fn main() {
    nui::run_app(CounterView::new());
}
