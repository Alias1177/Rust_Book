    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {

        for i in 0..nums.len(){
            for j in (i+1)..nums.len(){
                 if nums[i] + nums[j]==target{  
                    return vec![i as i32, j as i32];
                }
            }
        }
        vec![]
    }
 
    
fn shift_right(nums: &mut Vec<i32>, k: i32) {
    for _ in 0..k {
        let last = nums.pop();
            match last {
                Some(x) => nums.insert(0, x),
                None => break
            }
    }
}
    fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut i = 0;
        while i<nums.len(){
            if nums[i]==val{
                nums.swap_remove(i);
            }else{
                i+=1;
            }
        }
          nums.len() as i32
    }

fn remove_odds(nums: &mut Vec<i32>){
        for i in (0..nums.len()).rev() {
            if nums[i] % 2 == 0 {
                nums.remove(i);
            }
        }
    }

fn remove_duplicates(nums: &mut Vec<i32>){
        
        let mut i =1;
        while i<nums.len(){
            if nums[i]==nums[i-1]{
                nums.remove(i);
            }else{
                i+=1;
            }
        }
    }

fn remove_less_than(nums: &mut Vec<i32>, x: i32){
        let mut i =1;
        while i<nums.len(){
            if nums[i]<x{
                nums.swap_remove(i);
            }else{
                i+=1;
            }
        }
    }
    
fn take_just_uniqes(nums: &mut Vec<i32>) {
        nums.dedup();
    }



fn main() {
    let mut nums = vec![1, 2, 3, 4, 5, 6];
    shift_right(&mut nums, 1);
    println!("{:?}", nums);

    let mut nums = vec![1, 2, 3, 4, 5, 6];
    remove_odds(&mut nums);
    println!("{:?}", nums);

    let mut nums = vec![1, 1, 2, 3, 3, 4, 4, 5, 5];
    remove_duplicates(&mut nums);
    println!("{:?}", nums);

    let mut nums = vec![1, 2, 3, 4, 5, 6];
    remove_less_than(&mut nums, 3);
    println!("{:?}", nums);

    let mut nums = vec![1, 1, 2, 3, 3, 4, 5, 5];
    take_just_uniqes(&mut nums);
    println!("{:?}", nums);

    let mut nums = vec![2, 7, 11, 15];
    let res =remove_element(&mut nums, 9);
    println!("{:?}", res);
    println!("{:?}", nums);

}
