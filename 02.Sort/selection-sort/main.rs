struct SelectionSort{
    arr:[usize;13]
}

impl SelectionSort{
    fn new()->SelectionSort{
        SelectionSort{
            arr:[5,3,5,3,1,4,3,7,0,9,79,67,88]
        }
    }

    fn sort(&mut self)->(){

        // 가장 작은것 찾아서 순서대로
        let arr_size:usize = self.arr.len();
        for idx in 0..arr_size{
            let mut min:usize = idx;
            for jidx in idx+1..arr_size{
                println!("idx:{},jidx:{}",idx,jidx);
                if (SelectionSort::less(self.arr[jidx],self.arr[min])){
                    min = jidx;
                    println!("is larger");

                    let mut temp = self.arr[idx];
                    self.arr[idx] = self.arr[min];
                    self.arr[min] = temp;
                }

            }
        }

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
            if (SelectionSort::less(self.arr[i],self.arr[i-1])){
                return false;
            }
        }
        return true;
    }
}

fn main(){
    let mut selection_sort:SelectionSort = SelectionSort::new();
    let is_sorted:bool = selection_sort.is_sorted();
    let is_sorted = is_sorted.to_string();
    println!("initial is sorted:{}",is_sorted);
    selection_sort.show();
    selection_sort.sort();
    selection_sort.show();

}