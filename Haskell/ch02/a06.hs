{-# LANGUAGE TypeApplications #-}
{-# LANGUAGE FlexibleContexts #-}

{-# OPTIONS_GHC -Wno-unused-imports #-}
{-# OPTIONS_GHC -Wno-unused-top-binds #-}

import            Control.Monad (filterM, foldM, forM_, replicateM, when)
import            Control.Monad.ST (ST, runST)
import            Data.Array.IArray (bounds, listArray, (!), IArray, array)
import            Data.Array.MArray (newArray, readArray, writeArray, freeze)
import            Data.Array.ST (runSTUArray, STUArray)
import            Data.Array.Unboxed (UArray)
import            Data.Ix (Ix, inRange)
import            Data.Sequence (Seq, Seq(Empty, (:<|)), (|>))
import qualified  Data.Sequence as Seq
import qualified  Data.ByteString.Char8 as BS
import            Data.Char (isSpace)
import            Data.List (unfoldr, foldl)
import            Data.List.Extra (chunksOf)



getInts :: IO [Int]
getInts = unfoldr (BS.readInt . BS.dropWhile isSpace) <$> BS.getLine

cumulativeSum :: UArray Int Int -> Int -> UArray Int Int
cumulativeSum lst n = runSTUArray $ do
  sums <- newArray (0, n) 0
  forM_ [1 .. n] $ \i -> do
    prev <- readArray sums (i-1)
    writeArray sums i $ prev + lst!i
  return sums


main :: IO ()
main = do
  [n, q] <- getInts
  as <- listArray (1, n) <$> getInts :: IO (UArray Int Int)
  lrs <- replicateM q getInts

  let sums = cumulativeSum as n
  
  forM_ lrs $ \[left, right] ->
    print $ sums ! right - sums ! (left-1)


