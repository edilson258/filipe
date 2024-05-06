use core::fmt;

use super::object::Object;

#[derive(Clone, Debug)]
pub struct FilipeArray {
    inner: Vec<Object>,
    __cursor__: usize,
}

impl FilipeArray {
    pub fn new(init: Vec<Object>) -> Self {
        Self {
            inner: init,
            __cursor__: 0,
        }
    }

    pub fn push(&mut self, item: Object) {
        self.inner.push(item);
    }

    fn _next_item(&mut self) -> Option<Object> {
        if self.inner.len() <= self.__cursor__ {
            return None;
        }
        let item = self.inner[self.__cursor__].clone();
        self.__cursor__ += 1;
        Some(item)
    }
}

impl Iterator for FilipeArray {
    type Item = Object;
    fn next(&mut self) -> Option<Self::Item> {
        self._next_item()
    }
}

impl fmt::Display for FilipeArray {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        for (index, x) in self.inner.iter().enumerate() {
            if self.inner.len() - 1 == index {
                write!(f, "{}", x)?;
            } else {
                write!(f, "{}, ", x)?;
            }
        }
        write!(f, "]")
    }
}
