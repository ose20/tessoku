package ch04.b18;

import java.util.Arrays;
import java.util.List;
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
        boolean flg = IntStream.range(1, n+1).mapToObj(i -> dp[i][s]).reduce(false, (b1, b2) -> b1 || b2);
        if (flg) {
            var path = constructPath(n, s, a, dp);
            System.out.println(path.size());
            System.out.println(
                    path.stream().map(String::valueOf).collect(Collectors.joining(" "))
            );
        } else {
            System.out.println(-1);
        }
    }

    private static boolean[][] calcDp(int n, int s, int[] a) {
        boolean[][] dp = new boolean[n+1][s+1];
        for (int i=0; i<=n; i++) for (int j=0; j<=s; j++) dp[i][j] = false;
        dp[0][0] = true;

        for (int i=1; i<=n; i++) for (int j=0; j<=s; j++) {
            // i番目を使わない
            dp[i][j] = dp[i][j] || dp[i-1][j];
            // i番目を使う
            if (j >= a[i-1]) dp[i][j] = dp[i][j] || dp[i-1][j - a[i-1]];
        }

        return dp;
    }

    private static List<Integer> constructPath(int n, int s, int[] a, boolean[][] dp) {
        
        return null;
    }
}
