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
import            Data.Array.ST (runSTUArray, STUArray, runSTArray)
import            Data.Array.Unboxed (UArray)
import qualified  Data.ByteString.Char8 as BS
import            Data.Char (isSpace)
import            Data.Ix (Ix, inRange)
import            Data.List (unfoldr, foldl, sort)
import            Data.List.Extra (chunksOf, maximumOn)
import            Data.Sequence (Seq, Seq(Empty, (:<|)), (|>))
import qualified  Data.Sequence as Seq
import qualified  Data.Vector.Algorithms.Intro as VAI
import qualified  Data.Vector.Unboxed as VU
import qualified  Data.Vector.Unboxed.Mutable as VUM

getInts :: IO [Int]
getInts = unfoldr (BS.readInt . BS.dropWhile isSpace) <$> BS.getLine

maxv :: Int
maxv = 1_00_000

calcDp :: Int -> Array Int (Int, Int) -> Array (Int, Int) Int
calcDp n wvs = runSTArray $ do
  dp <- newArray ((0,0), (n, maxv)) inf
  -- init table
  writeArray dp (0, 0) 0

  -- construct table
  forM_ [1 .. n] $ \i -> do
    forM_ [0 .. maxv] $ \j -> do
      notUse <- readArray dp (i-1, j)
      if j - snd (wvs!i) >= 0 then do
        use <- readArray dp (i-1, j- snd (wvs!i))
        writeArray dp (i, j) $ min notUse $ use + fst (wvs!i)
      else do
        writeArray dp (i, j) notUse

  return dp

  where
    inf = 2_000_000_000 :: Int


calcAns :: Array (Int, Int) Int -> Int -> Int -> Int
calcAns dp n w =
  -- dp (n, 0) ~ dp (n, maxv) を取り出す
  -- dp(i, j) <= w 以下のものを切り出す
  -- 最大値をとる
  snd $ maximumOn snd $ filter (\(wi, _) -> wi <= w) table
  where
    table = iter 0 []
    iter i acc
      | i == maxv = (dp!(n,i), i) : acc
      | otherwise = iter (i+1) ((dp!(n,i), i) : acc)

main :: IO ()
main = do
  [n, w] <- getInts
  wvs <- listArray @Array (1,n) <$> 
    replicateM n (do
      [w, v] <- getInts
      return (w, v)
    )

  let dp = calcDp n wvs

  print $ calcAns dp n w
