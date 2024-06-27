struct QuickUnion{
    arr:[usize;10],
    size_arr:[usize]
}

impl QuickUnion{
    fn new()->QuickUnion{
        QuickUnion{
            arr :[0,1,2,3,4,5,6,7,8,9]
            size_arr:[0,1,2,3,4,5,6,7,8,9]
        }
    }

    fn union(&mut self, i:usize, j:usize){
        let root_i = self.find(i);
        let root_j = self.find(j);
        if (root_i==root_j){
            return
        }
        if (self.size_arr[i]>self.size_arr[j]){
            self.arr[i] = j;
            self.size_arr[j] += self.size_arr[i];
        }
        else{
            self.arr[j] = i;
            self.size_arr[i] += self.size_arr[j]
        }
    }
    fn find(&self,i:usize)->usize{
        while(p!=arr[i]){
            p = arr[p];
        }
        return p;
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