package ch02.B08;

import java.util.Scanner;

public class Main {
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);

        final int len = 1500;

        int n = sc.nextInt();
        int[][] x = new int[len+2][len+2];
        for (int i=0; i<n; i++) {
            int h = sc.nextInt();
            int w = sc.nextInt();
            x[h][w]++;
        }
        for (int i=1; i<=len; i++) {
            for (int j=1; j<=len; j++) {
                x[i][j] += (x[i-1][j] + x[i][j-1] - x[i-1][j-1]);
            }
        }

        int q = sc.nextInt();
        for (int i=0; i< q; i++) {
            int a = sc.nextInt();
            int b = sc.nextInt();
            int c = sc.nextInt();
            int d = sc.nextInt();

            int ans = x[c][d] - x[c][b-1] - x[a-1][d] + x[a-1][b-1];
            System.out.println(ans);
        }

    }
}
