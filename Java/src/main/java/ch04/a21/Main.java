package ch04.a21;

import java.util.Scanner;
import java.util.stream.IntStream;

public class Main {
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);

        int n = sc.nextInt();
        int[] p = new int[n];
        int[] a = new int[n];
        for (int i=0; i<n; i++) {
            p[i] = sc.nextInt();
            a[i] = sc.nextInt();
        }

        var dp = calcDp(n, p, a);
        System.out.println(
                IntStream.range(0, n+1).map(i -> dp[i][i+1]).max().orElse(-1)
        );
    }

    private static int[][] calcDp(int n, int[] p, int[] a) {
        int[][] dp = new int[n+2][n+2];

        // dp[i][j] := 左からi番目まで、右からj番目までを取り除いた状態での最高スコア
        // i=0は左からは何もとっていない、j=n+1は右からは何もとっていないことを表す
        for (int i=0; i<=n; i++) {
            for (int j=n+1; j>i; j--) {
                if (i > 0) {
                    // 左からi番目をとって(i,j)に遷移した場合
                    var tmp = dp[i-1][j] +
                            ((i+1 <= p[i-1] && p[i-1] <= j-1) ? a[i-1] : 0);
                    dp[i][j] = Math.max(dp[i][j], tmp);
                }
                if (j <= n) {
                    // 右からj番をとって(i,j)に遷移した場合
                    var tmp = dp[i][j+1] +
                            (i+1 <= p[j-1] && p[j-1] <= j-1 ? a[j-1] : 0);

                    dp[i][j] = Math.max(dp[i][j], tmp);
                }
            }
        }

        return dp;
    }

}
