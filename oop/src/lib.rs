pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            },
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_averaged_collection() {
        let add_values = vec![1, 2, 3, 4, 5, 6];
        let mut ac = AveragedCollection { list:vec![], average:0.0 };

        for value in &add_values {
            ac.add(*value);
        }
        assert_eq!((add_values.iter().sum::<i32>()) as f64 / add_values.len() as f64 , ac.average());

        let mut count = 0;
        while let Some(removed) = ac.remove() {
            count += 1;
            assert_eq!(removed, add_values[add_values.len() - count]);
        }
        assert_eq!(count, add_values.len());
    }
}
