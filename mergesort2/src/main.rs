fn main() {
	let mut list: Vec<u64> = vec![1_u64, 5, 2, 7, 3, 9, 4, 6];
	println!("unsorted list: {}", list);
	let sortedlist = mergesort(&mut list);
	println!("sorted list:   {}", sortedlist);
}

fn mergesort<T: Clone>(list: &mut Vec<T>) -> &mut Vec<T> {
	if list.len() <= 1 { 
		return list;
	}

	let (mut left, mut right) = list.partitioned(|&n| n > 0 && n < (list.len() as u64 /2));
	return list;
}
/*

public static List<T> Sort<T>(List<T> list) where T : IComparable
        {
            if (list.Count <= 1) return list;
 
            List<T> left = list.GetRange(0, list.Count / 2);
            List<T> right = list.GetRange(left.Count, list.Count - left.Count);
            return Merge(Sort(left), Sort(right));
        }
 
        public static List<T> Merge<T>(List<T> left, List<T> right) where T : IComparable
        {
            List<T> result = new List<T>();
            while (left.Count > 0 && right.Count > 0)
            {
                if (left[0].CompareTo(right[0]) <= 0)
                {
                    result.Add(left[0]);
                    left.RemoveAt(0);
                }
                else
                {
                    result.Add(right[0]);
                    right.RemoveAt(0);
                }
            }
            result.AddRange(left);
            result.AddRange(right);
            return result;
        }
*/
