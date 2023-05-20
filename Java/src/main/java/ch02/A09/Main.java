package ch02.A09;

import java.util.Scanner;

public class Main {
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);

        int h = sc.nextInt();
        int w = sc.nextInt();
        int n = sc.nextInt();

        int[][] snow = new int[h+2][w+2];
        for (int i=0; i<n; i++) {
            int a = sc.nextInt();
            int b = sc.nextInt();
            int c = sc.nextInt();
            int d = sc.nextInt();

            snow[a][b]++;
            snow[a][d+1]--;
            snow[c+1][b]--;
            snow[c+1][d+1]++;
        }

        for (int i=1; i<=h; i++) {
            for (int j=1; j<=w; j++) {
                snow[i][j] += snow[i][j-1];
            }
        }
        for (int j=1; j<=w; j++) {
            for (int i=1; i<=h; i++) {
                snow[i][j] += snow[i-1][j];
            }
        }

        for (int i=1; i<=h; i++) {
            for (int j=1; j<=w; j++) {
                System.out.printf("%d ", snow[i][j]);
            }
            System.out.println("");
        }

    }
}
