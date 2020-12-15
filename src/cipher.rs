pub struct CipherText {
    data : Vec<usize>
}

fn prop_holds(window : &[usize]) -> bool {
    let last = *window.last().unwrap();
    for i in 0..(window.len()-1) {
        for j in i..(window.len()-1) {
            if last == window[i] + window[j] {
                return true;
            }
        }
    }
    return false;
}

impl CipherText {
    pub fn from_lines<I>(lines : I) -> CipherText 
      where I : Iterator<Item = String> 
    {
        CipherText {
            data : lines.filter_map(
                |line| line.parse::<usize>().ok()
            ).collect()
        }
    }

    pub fn first_prop(self : &Self, block : usize) -> Option<usize> {
        self.data.windows(block+1).filter(
            |slice| !prop_holds(slice)
        ).map(
            |slice| slice[block]
        ).next()
    }

    pub fn weakness(self : &Self, block : usize) -> Option<usize> {
        let val = self.first_prop(block).unwrap();
        let len = self.data.len();
        for i in 0..(len-1) {
            for j in (i+1)..len {
                if val == self.data[i..=j].iter().sum::<usize>() {
                    let smallest = self.data[i..=j].iter().min().unwrap();
                    let largest = self.data[i..=j].iter().max().unwrap();
                    return Some(smallest + largest)
                }
            }
        }
        return None;
    }
}