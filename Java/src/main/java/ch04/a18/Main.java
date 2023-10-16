package ch04.a18;

import java.util.Scanner;
import java.util.stream.Collectors;
import java.util.stream.IntStream;

public class Main {
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);

        int n = sc.nextInt();
        int s = sc.nextInt();
        int[] a = IntStream.range(0, n).map(i -> sc.nextInt()).toArray();

        var dp = calcDp(n, s, a);

        System.out.println(
                IntStream.range(1, n+1).mapToObj(i -> dp[i][s]).reduce(false, (l, r) -> l || r) ? "Yes" : "No"
        );
    }

    private static boolean[][] calcDp(int n, int s, int[] a) {
        boolean[][] dp = new boolean[n+1][s+1];
        for (int i=0; i<=n; i++) for (int j=0; j<=s; j++) dp[i][j] = false;
        dp[0][0] = true;

        for (int i=1; i<=n; i++) {
            for (int j=0; j<=s; j++) {
                // i番目の数を使わない場合
                dp[i][j] = dp[i-1][j] || dp[i][j];
                // i番目の数を使う場合
                if (a[i-1] <= j) dp[i][j] = dp[i-1][j - a[i-1]] || dp[i][j];
            }
        }

        return dp;
    }
}
