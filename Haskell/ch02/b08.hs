{-# LANGUAGE TypeApplications #-}
{-# LANGUAGE FlexibleContexts #-}
{-# LANGUAGE TupleSections #-}

{-# OPTIONS_GHC -Wno-unused-imports #-}
{-# OPTIONS_GHC -Wno-unused-top-binds #-}

import            Control.Monad (filterM, foldM, forM_, replicateM, when)
import            Control.Monad.ST (ST, runST)
import            Data.Array (Array)
import            Data.Array.IArray (bounds, listArray, (!), IArray, accumArray, elems)
import            Data.Array.MArray (newArray, readArray, writeArray, freeze, thaw, newListArray, thaw)
import            Data.Array.ST (runSTUArray, STUArray)
import            Data.Array.Unboxed (UArray)
import            Data.Ix (Ix, inRange)
import            Data.Sequence (Seq, Seq(Empty, (:<|)), (|>))
import qualified  Data.Sequence as Seq
import qualified  Data.ByteString.Char8 as BS
import            Data.Char (isSpace)
import            Data.List (unfoldr, foldl, scanl1, scanl)
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

len :: Int
len = 1500

main :: IO ()
main = do
  n <- readLn @Int
  xys <- replicateM n $ do
    [x,y] <- getInts
    return (x, y)
  q <- readLn @Int
  queries <- replicateM q $ do
    [a,b,c,d] <- getInts
    return (a,b,c,d)

  let
    graph = elems @Array @[Int] $ accumArray (flip (:)) [] (0, len) xys
    rowCumu = map (scanl1 (+) . elems . accumArray @UArray @Int (+) 0 (0, len) . map ( , 1)) graph
    cumu = listArray @UArray ((0,0), (len,len)) $ concat $ scanl1 (zipWith (+)) rowCumu

  forM_ queries $ \(a,b,c,d) ->
    print $ cumu!(c,d) - cumu!(c,b-1) - cumu!(a-1,d) + cumu!(a-1,b-1)

