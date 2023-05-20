package ch02.B09;

import java.util.Scanner;

public class Main {
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);
        final int len = 1500;

        int n = sc.nextInt();
        int[][] origami = new int[len+2][len+2];
        for (int i=0; i<len+2; i++) for (int j=0; j<len+2; j++) origami[i][j] = 0;

        for (int i=0; i<n; i++) {
            int a = sc.nextInt();
            int b = sc.nextInt();
            int c = sc.nextInt();
            int d = sc.nextInt();

            origami[a][b]++;
            origami[a][d]--;
            origami[c][b]--;
            origami[c][d]++;
        }

        for (int i=0; i<=len; i++) {
            for (int j=1; j<=len; j++) {
                origami[i][j] += origami[i][j-1];
            }
        }

        for (int j=0; j<=len; j++) {
            for (int i=1; i<=len; i++) {
                origami[i][j] += origami[i-1][j];
            }
        }

        int area = 0;
        for (int i=0; i<=len; i++) for (int j=0; j<=len; j++) if (origami[i][j] > 0) area++;
        System.out.println(area);

//        for (int i=1; i<=10; i++) {
//            for (int j=1; j<=10; j++) {
//                System.out.printf("%d ", origami[i][j]);
//            }
//            System.out.println("");
//        }
    }
}
