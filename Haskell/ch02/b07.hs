{-# LANGUAGE TypeApplications #-}
{-# LANGUAGE FlexibleContexts #-}

{-# OPTIONS_GHC -Wno-unused-imports #-}
{-# OPTIONS_GHC -Wno-unused-top-binds #-}

import            Control.Monad (filterM, foldM, forM_, replicateM, when)
import            Control.Monad.ST (ST, runST)
import            Data.Array.IArray (bounds, listArray, (!), IArray, elems, accumArray)
import            Data.Array.MArray (newArray, readArray, writeArray, freeze)
import            Data.Array.ST (runSTUArray, STUArray)
import            Data.Array.Unboxed (UArray)
import            Data.Ix (Ix, inRange)
import            Data.Sequence (Seq, Seq(Empty, (:<|)), (|>))
import qualified  Data.Sequence as Seq
import qualified  Data.ByteString.Char8 as BS
import            Data.Char (isSpace)
import            Data.List (unfoldr, foldl, scanl1)
import            Data.List.Extra (chunksOf)



getInts :: IO [Int]
getInts = unfoldr (BS.readInt . BS.dropWhile isSpace) <$> BS.getLine

solve :: String -> Int
solve n =
  aux n 0
  where
    aux [] acc = acc
    aux ('1' : rest) acc = aux rest (2 * acc + 1)
    aux ('0' : rest) acc = aux rest (2 * acc)
    aux _ _ = error "cant occur"

main :: IO ()
main = do
  t <- readLn @Int
  n <- readLn @Int
  lrs <- replicateM n $ do
    [l, r] <- getInts
    return (l, r)

  let 
    tmp = accumArray @UArray @Int (+) 0 (0, t+1) $ concatMap (\(l, r) -> [(l, 1), (r, -1)]) lrs
    sums = scanl1 (+) $ elems tmp
  
  forM_ (init $ init sums) print

  