struct InsertionSort {
    arr:[usize;13]
}

impl InsertionSort {
    fn new()-> InsertionSort {
        InsertionSort {
            arr:[5,3,5,3,1,4,3,7,0,9,79,67,88]
        }
    }

    fn sort(&mut self)->(){
        // 하나씩 대조해서 정렬
        for i in 0..self.arr.len(){
            for j in (1..i+1).rev(){
                println!("index i:{},j:{} value:i:{}, j:{}",i,j,self.arr[i],self.arr[j]);
                if self.arr[j] < self.arr[j-1]{
                    self.exchange(j,j-1);


               }


            }
        }

    }
    fn exchange(&mut self,i:usize,j:usize){
        let temp:usize = self.arr[i];
        self.arr[i] = self.arr[j];
        self.arr[j] = temp;

    }
    fn less(i:usize, j:usize)->bool{
        return i<j;
    }
    fn show(&self){
        for i in 0..self.arr.len(){
            print!("{} ",self.arr[i])
        }
        println!("");
    }
    fn is_sorted(&self)->bool{
        for i in 1..self.arr.len(){
            if (InsertionSort::less(self.arr[i], self.arr[i-1])){
                return false;
            }
        }
        return true;
    }
}

fn main(){
    let mut selection_sort: InsertionSort = InsertionSort::new();
    let is_sorted:bool = selection_sort.is_sorted();
    let is_sorted = is_sorted.to_string();
    println!("initial is sorted:{}",is_sorted);
    selection_sort.show();
    selection_sort.sort();
    selection_sort.show();

}