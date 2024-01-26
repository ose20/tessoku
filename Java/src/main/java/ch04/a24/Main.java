package ch04.a24;

import java.util.Arrays;
import java.util.Scanner;

public class Main {
    private static int INF = 1_000_000_000;
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);

        int n = sc.nextInt();
        int[] a = new int[n];
        for (int i=0; i<n; i++) a[i] = sc.nextInt();

        var dp = calcDp(n, a);
        System.out.println(Arrays.stream(dp).max().orElse(-1));
    }

    private static int[] calcDp(int n, int[] a) {
        // dp[i] := i番目の値が最後尾になるような増加列のうち、最大の長さ
        // l[x] := 長さがxの増加部分列のうち、最後尾の値が取りうる最小値(dpの双対っぽい)
        // dpは副産物で、lを主体で考えたほうが直観的。図のわかりやすさは鉄則 < 螺旋
        int[] dp = new int[n+1];
        int[] l = new int[n+1];
        for (int i=1; i<=n; i++) l[i] = INF;

        for (int i=1; i<=n; i++) {
            int idx = Arrays.binarySearch(l, 0, i, a[i-1]);
            if (idx >= 0) {
                // a[i-1]が見つかった場合
                dp[i] = idx;
            } else {
                // a[i-1]が見つからない場合
                idx = (-idx - 1);
                dp[i] = idx;
                l[idx] = a[i-1];
            }
        }


        return dp;
    }
}
