{-# LANGUAGE BangPatterns #-}
{-# LANGUAGE FlexibleContexts #-}
{-# LANGUAGE NumericUnderscores #-}
{-# LANGUAGE TypeApplications #-}
{-# OPTIONS_GHC -Wno-name-shadowing #-}
{-# OPTIONS_GHC -Wno-unused-imports #-}
{-# OPTIONS_GHC -Wno-unused-top-binds #-}

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
import            Data.Ix (Ix, inRange, rangeSize)
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

lcs :: UArray Int Char -> UArray Int Char -> UArray (Int, Int) Int
lcs s t = runSTUArray $ do
  dp <- newArray ((0, 0), (slen, tlen)) 0

  forM_ [1 .. slen] $ \i ->
    forM_ [1 .. tlen] $ \j ->
      if s ! i == t ! j
        then do
          v <- readArray dp (i - 1, j - 1)
          writeArray dp (i, j) $ v + 1
        else do
          v1 <- readArray dp (i - 1, j)
          v2 <- readArray dp (i, j - 1)
          writeArray dp (i, j) $ max v1 v2

  return dp
  where
    slen = rangeSize $ bounds s
    tlen = rangeSize $ bounds t

main :: IO ()
main = do
  s <- getLine
  t <- getLine

  let s' = listArray @UArray (1, length s) s
      t' = listArray @UArray (1, length t) t

  print $ lcs s' t' ! (length s, length t)
