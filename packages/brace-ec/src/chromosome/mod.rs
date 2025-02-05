pub mod crossover;

pub trait Chromosome {
    type Gene;

    fn gene(&self, index: usize) -> Option<&Self::Gene>;

    fn gene_mut(&mut self, index: usize) -> Option<&mut Self::Gene>;

    fn len(&self) -> usize;

    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<T> Chromosome for Vec<T> {
    type Gene = T;

    fn gene(&self, index: usize) -> Option<&Self::Gene> {
        self.get(index)
    }

    fn gene_mut(&mut self, index: usize) -> Option<&mut Self::Gene> {
        self.get_mut(index)
    }

    fn len(&self) -> usize {
        self.len()
    }
}

impl<T, const N: usize> Chromosome for [T; N] {
    type Gene = T;

    fn gene(&self, index: usize) -> Option<&Self::Gene> {
        self.get(index)
    }

    fn gene_mut(&mut self, index: usize) -> Option<&mut Self::Gene> {
        self.get_mut(index)
    }

    fn len(&self) -> usize {
        N
    }
}

impl<T> Chromosome for [T] {
    type Gene = T;

    fn gene(&self, index: usize) -> Option<&Self::Gene> {
        self.get(index)
    }

    fn gene_mut(&mut self, index: usize) -> Option<&mut Self::Gene> {
        self.get_mut(index)
    }

    fn len(&self) -> usize {
        self.len()
    }
}
