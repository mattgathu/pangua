#!/usr/local/bin/Rscript

t <- read.table('values.dat', header=TRUE)
library(ggplot2)
ggplot(t, aes(n, comparisons, colour = algorithm)) + geom_point() + geom_smooth()
