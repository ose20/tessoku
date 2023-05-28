package ch02.A08;

import java.util.Scanner;

public class Main {
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);

        int h = sc.nextInt();
        int w = sc.nextInt();
        int[][] x = new int[h+2][w+2];
        for (int i=1; i<=h; i++) for (int j=1; j<=w; j++) x[i][j] = sc.nextInt();

        int q = sc.nextInt();

        imosInit(x, h, w);
        for (int i=0; i<q; i++) {
            int a = sc.nextInt();
            int b = sc.nextInt();
            int c = sc.nextInt();
            int d = sc.nextInt();
            procQuery(a, b, c, d, x);
        }

    }

    public static void imosInit(int[][] x, int h, int w) {
        for (int i=1; i<=h; i++) {
            for (int j=1; j<=w; j++) {
                x[i][j] += x[i-1][j];
                x[i][j] += x[i][j-1];
                x[i][j] -= x[i-1][j-1];
            }
        }
    }

    public static void procQuery(int a, int b, int c, int d, int[][] x) {
        int ans = x[c][d] - x[c][b-1] - x[a-1][d] + x[a-1][b-1];
        System.out.println(ans);
    }
}
