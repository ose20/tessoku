package ch04.b19;

import java.lang.reflect.Array;
import java.util.Arrays;
import java.util.Objects;
import java.util.Scanner;
import java.util.stream.IntStream;


public class Main {
    static final int maxv = 100*1000;
    static final long maxw = Long.MAX_VALUE / 4L;
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);

        int n = sc.nextInt();
        int w = sc.nextInt();

        long[] ws = new long[n];
        int[] vs = new int[n];

        for (int i=0; i<n; i++) {
            ws[i] = sc.nextInt();
            vs[i] = sc.nextInt();
        }

        var dp = calcDp(n, w, ws, vs);



        System.out.println(IntStream.range(0, maxv+1)
                .mapToObj(i -> Pair.of(dp[n][i], i))
                .filter(p -> p.fst <= w)
                .map(p -> p.snd)
                .reduce(0, Math::max)
        );



    }

    static long[][] calcDp(int n, int w, long[] ws, int[] vs) {
        // dp[i][j]: i番目までを使って、価値jを達成するのに必要な最小量
        var dp = new long[n+1][maxv+1];
        for (int i=0; i<=n; i++) for (int j=0; j<=maxv; j++) dp[i][j] = maxw;

        dp[0][0] = 0;

        for (int i=1; i<=n; i++) for (int j=0; j<=maxv; j++) {
            // 使わない場合
            dp[i][j] = Math.min(dp[i][j], dp[i-1][j]);
            // 使う場合
            if (j - vs[i-1] >= 0) dp[i][j] = Math.min(dp[i][j], dp[i-1][j - vs[i-1]] + ws[i-1]);
        }

        return dp;
    }
}


class Pair<F extends Comparable<F>, S extends Comparable<S>> implements Comparable<Pair<F, S>> {
    public final F fst;
    public final S snd;

    private Pair(F fst, S snd) {
        this.fst = fst;
        this.snd = snd;
    }

    public static <F extends Comparable<F>, S extends Comparable<S>> Pair<F, S> of(F fst, S snd) {
        return new Pair<>(fst, snd);
    }

    @Override
    public int compareTo(Pair<F, S> o) {
        int cmpFst = this.fst.compareTo(o.fst);
        return cmpFst != 0 ? cmpFst : this.snd.compareTo(o.snd);
    }

    @Override
    public boolean equals(Object o) {
        if (this == o) return true;
        if (o == null || getClass() != o.getClass()) return false;
        Pair<?, ?> pair = (Pair<?, ?>) o;
        return Objects.equals(fst, pair.fst) && Objects.equals(snd, pair.snd);
    }

    @Override
    public String toString() {
        return String.format("Pair{ %s, %s }", this.fst, this.snd);
    }

    @Override
    public int hashCode() {
        return Objects.hash(fst, snd);
    }
}