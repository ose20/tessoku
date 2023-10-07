{-# LANGUAGE TypeApplications #-}
{-# LANGUAGE FlexibleContexts #-}
{-# LANGUAGE NumericUnderscores #-}

{-# OPTIONS_GHC -Wno-unused-imports #-}
{-# OPTIONS_GHC -Wno-unused-top-binds #-}

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
import qualified  Data.Vector.Unboxed as VU
import Data.ByteString (ByteString)


getInts :: IO [Int]
getInts = unfoldr (BS.readInt . BS.dropWhile isSpace) <$> BS.getLine

binSearch :: (Int, Int) -> (Int -> Bool) -> Int
binSearch (left, right) p =
  loop leftSentinel rightSentinel
  where
    leftSentinel = left+1
    rightSentinel = right-1
    loop ok ng
      | ok-1 == ng = ok
      | mid == leftSentinel = mid
      | mid == rightSentinel = mid
      | otherwise = if p mid then loop mid ng else loop ok mid
        where
          mid = (ok + ng) `div` 2

main :: IO ()
main = do
  [_, k] <- getInts
  as <- getInts

  print $ binSearch (1000_000_000, 0) (\time -> sum (map (div time) as) >= k)
