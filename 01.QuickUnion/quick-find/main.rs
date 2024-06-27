struct UnionFind{
    arr:[usize;10]

}
impl UnionFind{
    pub fn new()->UnionFind{
        UnionFind{
            arr :[0,1,2,3,4,5,6,7,8,9]
        }
    }
    fn connected(&self,i:usize,j:usize)->bool{
        if self.arr[i]==self.arr[j] {
            return true
        }
        return false
    }

    fn union(&mut self,i:usize,j:usize){
        if (self.arr[i]==self.arr[j]){
            return;
        }
        let i_val = self.arr[i];
        let j_val = self.arr[j];
        for index in 0..10{
            if (i_val==self.arr[index]){
                self.arr[index] = self.arr[j];
            }

        }

    }
    fn print(&self){
        for idx in 0..10{
            print!("{}",self.arr[idx]);
        }
        println!("");
    }
}

fn main(){
    let mut uf:UnionFind = UnionFind::new();
    uf.union(3,4);
    uf.print();
    uf.union(3,5);
    uf.print();

}