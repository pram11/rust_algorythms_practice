struct QuickUnion{
    arr:[usize;10],
}

impl QuickUnion{
    fn new()->QuickUnion{
        QuickUnion{
            arr :[0,1,2,3,4,5,6,7,8,9]
        }
    }
    fn root(&self,mut i:usize)->usize{
        while i!=self.arr[i] {
            i = self.arr[i];
        }
        return i;
    }
    fn union(&mut self, i:usize, j:usize){
        let root_i = self.root(i);
        let root_j = self.root(j);
        self.arr[i] = j;
    }
    fn connected(&self,i:usize,j:usize)->bool{
        return self.root(i) == self.root(j);
    }
}

fn main(){
    let mut qu:QuickUnion = QuickUnion::new();
    qu.union(1,2);
    let is_connected =  qu.connected(1,2);
    let is_connected = is_connected.to_string();

    println!("{}",is_connected);
}