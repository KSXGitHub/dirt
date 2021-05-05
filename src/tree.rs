use super::size::Size;
use rayon::prelude::*;
use std::cmp::Ordering;

/// Disk usage data of a filesystem tree.
#[derive(Debug, PartialEq, Eq)]
pub struct Tree<Name, Data: Size> {
    name: Name,
    data: Data,
    children: Vec<Self>,
}

impl<Name, Data: Size> Tree<Name, Data> {
    /// Extract name
    pub fn name(&self) -> &Name {
        &self.name
    }

    /// Extract total disk usage
    pub fn data(&self) -> Data {
        self.data
    }

    /// Extract children
    pub fn children(&self) -> &Vec<Self> {
        &self.children
    }

    /// Create a tree representation of a directory.
    pub fn dir(name: Name, inode_size: Data, children: Vec<Self>) -> Self {
        let data = inode_size + children.iter().map(Tree::data).sum();
        Tree {
            name,
            data,
            children,
        }
    }

    /// Create a tree representation of a file.
    pub fn file(name: Name, data: Data) -> Self {
        Tree {
            name,
            data,
            children: Vec::with_capacity(0),
        }
    }

    /// Create a directory constructor of fixed inode size.
    pub fn fixed_size_dir_constructor(inode_size: Data) -> impl Fn(Name, Vec<Self>) -> Self
    where
        Data: Copy,
    {
        move |name, children| Tree::dir(name, inode_size, children)
    }

    /// Create reflection.
    pub fn into_reflection(self) -> TreeReflection<Name, Data> {
        self.into()
    }

    /// Sort all descendants recursively, in parallel.
    pub fn par_sort_by(&mut self, compare: impl Fn(&Self, &Self) -> Ordering + Copy + Sync)
    where
        Self: Send,
    {
        self.children
            .par_iter_mut()
            .for_each(|child| child.par_sort_by(compare));
        self.children.sort_by(compare);
    }

    /// Process the tree via [`par_sort_by`](Self::par_sort_by) method.
    pub fn into_par_sorted(
        mut self,
        compare: impl Fn(&Self, &Self) -> Ordering + Copy + Sync,
    ) -> Self
    where
        Self: Send,
    {
        self.par_sort_by(compare);
        self
    }
}

/// Reflection of [`Tree`] used for testing purposes.
///
/// Unlike `Tree` where the fields are all private, the fields of `TreeReflection`
/// are all public to allow construction in tests.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeReflection<Name, Data: Size> {
    /// Name of the tree.
    pub name: Name,
    /// Disk usage of a file or total disk usage of a folder.
    pub data: Data,
    /// Data of children filesystem subtrees.
    pub children: Vec<Self>,
}

impl<Name, Data: Size> From<Tree<Name, Data>> for TreeReflection<Name, Data> {
    fn from(source: Tree<Name, Data>) -> Self {
        let Tree {
            name,
            data,
            children,
        } = source;
        let children: Vec<_> = children.into_iter().map(TreeReflection::from).collect();
        TreeReflection {
            name,
            data,
            children,
        }
    }
}
