pub mod either;
pub mod evaluate;
pub mod evaluator;
pub mod evolver;
pub mod generator;
pub mod inspect;
pub mod mutator;
pub mod recombinator;
pub mod repeat;
pub mod selector;
pub mod then;
pub mod weighted;

use self::either::Either;

pub trait IntoParallelOperator: Sized {
    type Op;

    fn parallel(self) -> Self::Op;

    fn parallel_if(self, parallel: bool) -> Either<Self, Self::Op> {
        match parallel {
            false => Either::A(self),
            true => Either::B(self.parallel()),
        }
    }
}
