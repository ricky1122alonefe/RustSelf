fn main() {
    let y = while_true(5);
    assert_eq!(y,6 );
}

/**
 下面代码在编译器中 循环条件可真可假 只知道返回单元值 CTFE 无法作为编译器的常量使用
 同理 if true 在只有一条分之情况下 也会出现类似问题

fn while_true(x :i32) -> i32{
    while true{
        return x+1;
    }
}
 */

 // 修复代码 后 可以进行变异 让编译器理解  while_true 返回值类型是i32类型 当然  只会在 while true 中循环执行
 fn while_true(x :i32)-> i32{
     while true{
         return x+1
     }
     x 
 }