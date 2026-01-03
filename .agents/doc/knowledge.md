# Knowledge

* `Error::source` returns `Option<&(dyn Error + 'static)>`, which doesn't implement `Errgonomic`, so we can't call `Errgonomic::fmt` on it
* The user wants to see the full error chain
  * We must descend down the chain
    * #options
      * We can descend using `Error::source`
      * We can descend using `Errgonomic::source`
  * We can call `downcast_ref` on `dyn Error`
