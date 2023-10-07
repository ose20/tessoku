{-# LANGUAGE TypeApplications #-}
{-# LANGUAGE FlexibleContexts #-}
{-# LANGUAGE NumericUnderscores #-}
{-# LANGUAGE BangPatterns #-}

{-# OPTIONS_GHC -Wno-unused-imports #-}
{-# OPTIONS_GHC -Wno-unused-top-binds #-}
{-# OPTIONS_GHC -Wno-name-shadowing #-}

import            Control.Monad (filterM, foldM, forM_, replicateM, when)
import            Control.Monad.ST (ST, runST)
import            Data.Array (Array)
import            Data.Array.IArray (bounds, listArray, (!), IArray, accumArray, elems)
import            Data.Array.MArray (newArray, readArray, writeArray, freeze)
import            Data.Array.ST (runSTUArray, STUArray)
import            Data.Array.Unboxed (UArray)
import qualified  Data.ByteString.Char8 as BS
import            Data.Char (isSpace)
import            Data.Ix (Ix, inRange)
import            Data.List (unfoldr, foldl, sort)
import            Data.List.Extra (chunksOf)
import            Data.Sequence (Seq, Seq(Empty, (:<|)), (|>))
import qualified  Data.Sequence as Seq
import qualified  Data.Vector.Algorithms.Intro as VAI
import qualified  Data.Vector.Unboxed as VU
import qualified  Data.Vector.Unboxed.Mutable as VUM

getInts :: IO [Int]
getInts = unfoldr (BS.readInt . BS.dropWhile isSpace) <$> BS.getLine

calcDp :: Int -> Array Int (Int, Int) -> UArray (Int, Int) Int
calcDp n pas = runSTUArray $ do
  dp <- newArray ((1, 1), (n, n)) 0

  forM_ [1 .. n] $ \i ->
    forM_ (reverse [1 .. n]) $ \j -> do
      when (i-1 >= 1) $ do
        v <- readArray dp (i-1, j)
        tmp <- readArray dp (i, j)
        writeArray dp (i, j) $ max tmp $
          v + if i <= fst (pas!(i-1)) && fst (pas!(i-1)) <= j then snd (pas!(i-1)) else 0
      when (j+1 <= n) $ do
        v <- readArray dp (i, j+1)
        tmp <- readArray dp (i, j)
        writeArray dp (i, j) $ max tmp $
          v + if i <= fst (pas!(j+1)) && fst (pas!(j+1)) <= j then snd (pas!(j+1)) else 0

  return dp 

main :: IO ()
main = do
  n <- readLn @Int
  pas <- listArray @Array (1, n) <$> replicateM n (do
    [p, a] <- getInts
    return (p, a))

  let dp = calcDp n pas

  print $ maximum [ dp ! (i, i) | i <- [1 .. n] ]

