package ch04.a23;

import java.util.Arrays;
import java.util.Scanner;

public class Main {

    static final int maxv = 1000_000_000;
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);

        int n = sc.nextInt();
        int m = sc.nextInt();

        int[][] a = new int[m][n];
        for (int i=0; i<m; i++) for (int j=0; j<n; j++) a[i][j] = sc.nextInt();

        var coupon = calcCoupon(n, m, a);
        //Arrays.stream(coupon).forEach(System.out::println);
        var dp = calcDp(n, m, coupon);

        System.out.println(
                (dp[(1<<n) - 1] == maxv ? -1 : dp[(1<<n) - 1])
        );
    }

    // m個のクーポンをbit表記にする
    private static int[] calcCoupon(int n, int m, int[][] a) {
        int[] coupon = new int[m];

        for (int i=0; i<m; i++) {
            int base = 1;
            for (int j=n-1; j>=0; j--) {
                coupon[i] += a[i][j] * base;
                base *= 2;
            }
        }

        return coupon;
    }

    private static int[] calcDp(int n, int m, int[] coupon) {
        // dp[i]: iをビット表現した時、立ってるビットに対応するお店だけを半額にするために必要なクーポンの最小枚数
        int[] dp = new int[1<<n];
        for (int i=1; i<(1<<n); i++) dp[i] = maxv;

        for (int bit=0; bit<(1<<n); bit++) {
            if (dp[bit] == maxv) continue;
            for (int i=0; i<m; i++) {
                // 使うクーポンでるーぷ
                var tar = bit | coupon[i];
                //System.out.printf("(bit, i, tar) = %d, %d, %d\n", bit, i, tar);
                dp[tar] = Math.min(dp[tar], dp[bit] + 1);
                //if (dp[tar] == 1 && tar == 7) System.out.printf("!!! (bit, i, tar) = %d, %d, %d\n", bit, i, tar);
            }
        }

        return dp;
    }
}
