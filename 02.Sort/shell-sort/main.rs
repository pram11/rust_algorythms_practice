struct ShellSort {
    arr:[usize;13]
}

impl ShellSort {
    fn new()-> ShellSort {
        ShellSort {
            arr:[5,3,5,3,1,4,3,7,0,9,79,67,88]
        }
    }

    fn sort(&mut self)->(){
        // 기준 숫자로 분할, 정렬
        // 기준숫자: 여러 제안이 있음. knuth의 3x+1이 가장 보편적?
        let N:usize = self.arr.len();
        let mut h:usize = 1;
        while h<(N/3) {
            h = 3 * h + 1;
        }
        println!("h:{}",h);
        while h>=1{
            for i in h..N{
                let mut j:usize = i;
                while j>=h && ShellSort::less(self.arr[j],self.arr[j-h]){
                    self.exchange(j,j-h);
                    println!("i:{}, j:{},h:{}",i,j,h);
                    j= j-h;
                }
            }
            h = h/3;
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
            if (ShellSort::less(self.arr[i], self.arr[i-1])){
                return false;
            }
        }
        return true;
    }
}

fn main(){
    let mut selection_sort: ShellSort = ShellSort::new();
    let is_sorted:bool = selection_sort.is_sorted();
    let is_sorted = is_sorted.to_string();
    println!("initial is sorted:{}",is_sorted);
    selection_sort.show();
    selection_sort.sort();
    selection_sort.show();

}