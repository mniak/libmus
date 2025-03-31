pub trait IntoGroupsExt<T>: Iterator<Item = T> + Sized {
    fn into_groups<F>(self, threshold: f32, get_value: F) -> IntoGroups<T, Self, F>
    where
        F: Fn(&T) -> f32,
    {
        IntoGroups {
            iter: self,
            threshold,
            get_value,
        }
    }
}

impl<T, I> IntoGroupsExt<T> for I where I: Iterator<Item = T> {}

pub struct IntoGroups<T, I, F>
where
    I: Iterator<Item = T>,
    F: Fn(&T) -> f32,
{
    iter: I,
    threshold: f32,
    get_value: F,
}

impl<T, I, F> Iterator for IntoGroups<T, I, F>
where
    I: Iterator<Item = T>,
    F: Fn(&T) -> f32,
{
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut accumulator: f32 = 0.0;
        let mut group = Vec::new();

        while let Some(d) = self.iter.next() {
            accumulator += (&self.get_value)(&d);
            group.push(d);

            if accumulator >= self.threshold {
                return Some(group);
            }
        }

        match group.is_empty() {
            true => None,
            false => Some(group),
        }
    }
}
