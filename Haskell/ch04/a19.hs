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
import            Data.Bifunctor (bimap)
import qualified  Data.ByteString.Char8 as BS
import            Data.Char (isSpace)
import            Data.Ix (Ix, inRange)
import            Data.List (unfoldr, foldl, sort)
import            Data.List.Extra (chunksOf)
import            Data.Sequence (Seq, Seq(Empty, (:<|)), (|>))
import qualified  Data.Sequence as Seq
import qualified  Data.Vector as V
import qualified  Data.Vector.Algorithms.Intro as VAI
import qualified  Data.Vector.Mutable as VM
import qualified  Data.Vector.Unboxed as VU
import qualified  Data.Vector.Unboxed.Mutable as VUM

getInts :: IO [Int]
getInts = unfoldr (BS.readInt . BS.dropWhile isSpace) <$> BS.getLine

calcDp :: Int -> Int -> UArray Int Int -> UArray Int Int -> UArray (Int, Int) Int
calcDp n w ws vs = runSTUArray $ do
  dp <- newArray ((0, 0), (n, w)) 0

  forM_ [1 .. n] $ \i ->
    forM_ [1 .. w] $ \j -> 
      if j - ws!i >= 0 then do
        v1 <- readArray dp (i-1, j)
        v2 <- readArray dp (i-1, j - ws!i)
        writeArray dp (i, j) $ max v1 (v2 + vs!i)
      else do
        v <- readArray dp (i-1, j)
        writeArray dp (i, j) v

  return dp

main :: IO ()
main = do
  [n, w] <- getInts
  wvs <- replicateM n $ do
    [w, v] <- getInts
    return (w, v)

  let
    (ws, vs) = bimap (listArray @UArray (1, n)) (listArray @UArray (1, n)) $ unzip wvs
    dp = calcDp n w ws vs

  print $ maximum [ dp ! (n, i) | i <- [0 .. w] ]


