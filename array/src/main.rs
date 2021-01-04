fn main() {
   let arr:[i32;3] = [1,2,3];
   let mut mut_arr = [1,2,3];
   assert_eq!(1,mut_arr[0] );
   mut_arr[0] = 3;
   assert_eq!(3,mut_arr[0]);
   
}

 /**
 [T;N] N个值的数组 每个类型都为t 编译时间确定的常量
 Vec<T> T类型响亮 动态分配 可扩展 保存堆 可以缩放
 &[T] 类型共享切片  &mut[T] 可修改切片
  */