{-# LANGUAGE TypeApplications #-}
{-# LANGUAGE FlexibleContexts #-}
{-# LANGUAGE NumericUnderscores #-}

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
import qualified  Data.Vector.Unboxed as VU
import Data.ByteString (ByteString)



getInts :: IO [Int]
getInts = unfoldr (BS.readInt . BS.dropWhile isSpace) <$> BS.getLine

binSearch :: (Int, Int) -> (Int -> Bool) -> Int
binSearch (left, right) p =
  loop leftSentinel rightSentinel
  where
    leftSentinel = left-1
    rightSentinel = right+1
    loop left right
      | left+1 == right = left
      | mid == leftSentinel = mid
      | mid == rightSentinel = mid
      | otherwise = if p mid then loop mid right else loop left mid
          where mid = (left + right) `div` 2

main :: IO ()
main = do
  [n, k] <- getInts
  as <- VU.fromList <$> getInts

  print $ sum $ map 
    ( \i -> 
        let j = binSearch 
                  (i+1, n-1) 
                  (\j -> as VU.! j - as VU.! i <= k)
        in j - i
    ) 
    [0 .. n-1]

