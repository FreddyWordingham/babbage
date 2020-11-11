(function() {var implementors = {};
implementors["ndarray"] = [{"text":"impl&lt;'a, A, D&gt; IndexedParallelIterator for Parallel&lt;AxisIter&lt;'a, A, D&gt;&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;D: Dimension,<br>&nbsp;&nbsp;&nbsp;&nbsp;A: Sync,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;'a, A, D&gt; IndexedParallelIterator for Parallel&lt;AxisIterMut&lt;'a, A, D&gt;&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;D: Dimension,<br>&nbsp;&nbsp;&nbsp;&nbsp;A: Send + Sync,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;'a, A, D&gt; IndexedParallelIterator for Parallel&lt;AxisChunksIter&lt;'a, A, D&gt;&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;D: Dimension,<br>&nbsp;&nbsp;&nbsp;&nbsp;A: Sync,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;'a, A, D&gt; IndexedParallelIterator for Parallel&lt;AxisChunksIterMut&lt;'a, A, D&gt;&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;D: Dimension,<br>&nbsp;&nbsp;&nbsp;&nbsp;A: Send + Sync,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["ndarray_parallel"] = [{"text":"impl&lt;'a, A, D&gt; IndexedParallelIterator for Parallel&lt;AxisIter&lt;'a, A, D&gt;&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;D: Dimension,<br>&nbsp;&nbsp;&nbsp;&nbsp;A: Sync,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;'a, A, D&gt; IndexedParallelIterator for Parallel&lt;AxisIterMut&lt;'a, A, D&gt;&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;D: Dimension,<br>&nbsp;&nbsp;&nbsp;&nbsp;A: Send + Sync,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["rayon"] = [];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()