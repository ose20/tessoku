{-# LANGUAGE TypeApplications #-}
{-# LANGUAGE FlexibleContexts #-}
{-# LANGUAGE NumericUnderscores #-}
{-# LANGUAGE BangPatterns #-}

{-# OPTIONS_GHC -Wno-unused-imports #-}
{-# OPTIONS_GHC -Wno-unused-top-binds #-}
{-# OPTIONS_GHC -Wno-name-shadowing #-}

import            Control.Monad (filterM, foldM, forM_, replicateM, when, forM)
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

dp :: Int -> UArray Int Int -> UArray Int Int -> UArray Int Int
dp n as bs = runSTUArray $ do
  arr <- newArray (1, n) inf
  writeArray arr 1 0

  forM_ [2 .. n] $ \i ->
    if i <= 2 then do
      prev <- readArray arr (i-1)
      writeArray arr i $ prev + as ! i
    else do
      prev <- readArray arr (i-1)
      prev2 <- readArray arr (i-2)
      writeArray arr i $ min (prev + as!i) (prev2 + bs!i)

  return arr

  where
    inf = 1_000_000_000 :: Int

rebuildPath :: UArray Int Int -> Int -> UArray Int Int -> UArray Int Int -> [Int]
rebuildPath dp n a b =
  loop n [n]
  where
    loop i acc
      | i == 1 = 1:acc
      | i == 2 = loop 1 (2:acc)
      | otherwise =
          if step1 <= step2 then loop (i-1) $ (i-1):acc else loop (i-2) $ (i-2):acc
        where
          step1 = dp!(i-1) + a!i
          step2 = dp!(i-2) + b!i


main :: IO ()
main = do
  n <- readLn @Int
  as <- listArray @UArray (2, n) <$> getInts
  bs <- listArray @UArray (3, n) <$> getInts

  let ans = rebuildPath (dp n as bs) n as bs

  print $ length ans
  putStrLn $ unwords $ map show ans