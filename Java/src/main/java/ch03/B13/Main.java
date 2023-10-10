package ch03.B13;

import java.util.Scanner;
import java.util.stream.IntStream;
import java.util.stream.LongStream;

public class Main {
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);

        int n = sc.nextInt();
        int k = sc.nextInt();
        int[] a = IntStream.range(0, n).map(i -> sc.nextInt()).toArray();

        solve(n, k, a);
    }

    private static void solve(int n, int k, int[] a) {
           long[] cum = getCum(n, a);

           long ans = 0;
           int right = 1;
           // [left, right]
           for (int left=1; left<=n; left++) {
               right = Math.max(right, left);

               while (right <= n && (cum[right] - cum[left-1]) <= k) { right++; }
               ans += right - left;
           }

           System.out.println(ans);
    }


    private static long[] getCum(int n, int[] a) {
        long[] cum = LongStream.range(0, n+1).map(val -> 0L).toArray();
        IntStream.range(1, n+1).forEach(i -> cum[i] = cum[i-1] + a[i-1]);
        return cum;
    }
}
