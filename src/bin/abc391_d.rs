use proconio::input;

fn main() {
    input! {
        n: usize,
        w: usize,
    }

    let mut block = Vec::new();
    let mut ws = vec![vec![]; w];
    for _ in 0..n {
        input!{ xi: usize, yi: usize}
        ws[xi-1].push(yi);
        block.push(ws[xi-1].len())
    }

    let mut delete_block_num = ws[0].len();
    for i in 1..w {
        delete_block_num = delete_block_num.min(ws[i].len())
    }

    // delete_t = [1, 3, 5] であれば
    //   t=1に下から1つ目のブロックが削除
    //   t=3に下から2つ目のブロックが削除
    //   t=5に下から3つ目のブロックが削除
    let mut delete_t = Vec::new();
    for i in 0..delete_block_num {
        let mut dt = 0;
        for wi in 0..w {
            dt = dt.max(ws[wi][i]);
        }
        delete_t.push(dt)
    }

    // print!("delete_t");
    // for dt in &delete_t {
    //     print!(" {}", *dt);
    // }
    // println!();

    input!{ q: usize }
    for _ in 0..q {
        input!{ t: usize, mut a: usize }
        a -= 1;
        let block_num = block[a]; // 下から何番目か
        // print!("a:{},block_num:{} ", a, block_num);
        if block_num-1 < delete_t.len() {
            if delete_t[block_num-1] <= t {
                println!("No");
                continue
            }
        }
        println!("Yes")
    }
}