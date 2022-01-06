use proconio::input;

fn binary_search(arr: &[i32], l: i32, r: i32, x: i32) -> i32 {
    if r >= l {
        let mid = (l + r) / 2;
        if arr[mid as usize] == x {
            return mid as i32;
        } else if arr[mid as usize] > x {
            return binary_search(arr, l, mid - 1, x);
        } else {
            return binary_search(arr, mid + 1, r, x);
        }
    } else {
        return -1;
    }
}

fn main() {
    // 探したい数字を入力
    input! {
        n: i32,
    }
    // 探索範囲となるARRAY
    let arr: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let left = 0;
    let right = 10;
    let ans = binary_search(&arr, left, right, n);

    if ans == -1 {
        println!("1から10の値nを入力してください");
    } else {
        println!("nが格納されているindexは {} です", ans);
    }
}
