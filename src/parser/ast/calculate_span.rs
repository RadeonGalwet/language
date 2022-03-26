use crate::common::span::Span;

pub trait CalculateSpan {
    fn calculate_span(&self) -> Span;
}
