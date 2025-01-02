use newtype::NewType;
use pest::iterators::{FlatPairs as InnerFlatPairs, Pair as InnerPair, Pairs as InnerPairs};
use trql::tree::{FlatTree, Node, Tree};

#[derive(NewType, Debug, Clone, Hash, PartialEq, Eq)]
pub struct Pair<'i>(pub InnerPair<'i, &'i str>);

impl<'i> Node for Pair<'i> {
    type Tree = Pairs<'i>;
    type FlatTree = FlatPairs<'i>;

    fn name(&self) -> &str {
        self.as_rule()
    }

    fn value(&self) -> Option<&str> {
        Some(self.as_str())
    }

    fn tree(self) -> Self::Tree {
        Pairs(self.0.into_inner())
    }

    fn flat_tree(self) -> Self::FlatTree {
        FlatPairs(self.0.into_inner().flatten())
    }
}

#[derive(NewType, Debug, Clone, Hash, PartialEq, Eq)]
pub struct Pairs<'i>(pub InnerPairs<'i, &'i str>);

impl<'i> Iterator for Pairs<'i> {
    type Item = Pair<'i>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|p| Pair(p))
    }
}

impl<'i> Tree for Pairs<'i> {
    type Node = Pair<'i>;
}

#[derive(NewType, Debug, Clone)]
pub struct FlatPairs<'i>(pub InnerFlatPairs<'i, &'i str>);

impl<'i> Iterator for FlatPairs<'i> {
    type Item = Pair<'i>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|p| Pair(p))
    }
}

impl<'i> FlatTree for FlatPairs<'i> {
    type Node = Pair<'i>;
}
