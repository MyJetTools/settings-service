#[derive(Debug, Clone, Copy)]
pub enum ProductId<'s> {
    Shared,
    Id(&'s str),
}

impl<'s> ProductId<'s> {
    pub fn try_unwrap_id(&self) -> Option<&str> {
        match self {
            Self::Id(value) => Some(value),
            Self::Shared => None,
        }
    }
}

impl<'s> Into<ProductId<'s>> for &'s str {
    fn into(self) -> ProductId<'s> {
        if self.is_empty() {
            ProductId::Shared
        } else {
            ProductId::Id(self)
        }
    }
}

impl<'s> Into<ProductId<'s>> for Option<&'s str> {
    fn into(self) -> ProductId<'s> {
        match self {
            Some(value) if !value.is_empty() => ProductId::Id(value),
            _ => ProductId::Shared,
        }
    }
}
