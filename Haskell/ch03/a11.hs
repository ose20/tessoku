{-# LANGUAGE TypeApplications #-}
{-# LANGUAGE FlexibleContexts #-}

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
import            Data.List (unfoldr, foldl)
import            Data.List.Extra (chunksOf)
import            Data.Sequence (Seq, Seq(Empty, (:<|)), (|>))
import qualified  Data.Sequence as Seq
import qualified  Data.Vector.Unboxed as VU
import Data.ByteString (ByteString)


getInts :: IO [Int]
getInts = unfoldr (BS.readInt . BS.dropWhile isSpace) <$> BS.getLine

binSearch :: (Int, Int) -> (Int -> Bool) -> Int
binSearch (left, right) p =
  let ok = left - 1
      ng = right + 1
  in
  loop ok ng
  where
    loop ok ng
      | ok + 1 == ng = ok
      | otherwise = if p mid then loop mid ng else loop ok mid
          where
            mid = (ok + ng) `div` 2
    

main :: IO ()
main = do
  [n, x] <- getInts
  as <- VU.fromList <$> getInts :: IO (VU.Vector Int)

  print $ binSearch (0, n-1) (\i -> as VU.! i <= x) + 1
