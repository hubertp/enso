package org.enso.interpreter.node.expression.builtin.date;

import com.oracle.truffle.api.CompilerDirectives;
import com.oracle.truffle.api.dsl.Specialization;
import com.oracle.truffle.api.interop.InteropLibrary;
import com.oracle.truffle.api.interop.UnsupportedMessageException;
import com.oracle.truffle.api.library.CachedLibrary;
import com.oracle.truffle.api.nodes.Node;
import org.enso.interpreter.dsl.BuiltinMethod;
import org.enso.interpreter.dsl.MonadicState;

@BuiltinMethod(
    type = "Date_Internal",
    name = "date_value",
    description = "Returns some value for a Date")
public abstract class YearMonthDayNode extends Node {
  public static YearMonthDayNode build() {
    return YearMonthDayNodeGen.create();
  }

  abstract long execute(@MonadicState Object state, Object self, long type, Object date);

  @Specialization(guards = "type == 1")
  long executeYear(
      Object state,
      Object self,
      long type,
      Object date,
      @CachedLibrary(limit="3") InteropLibrary iop
  ) {
    try {
      return iop.asDate(date).getYear();
    } catch (UnsupportedMessageException ex) {
      CompilerDirectives.transferToInterpreter();
      throw new IllegalStateException(ex);
    }
  }

  @Specialization(guards = "type == 2")
  long executeMonth(
      Object state,
      Object self,
      long type,
      Object date,
      @CachedLibrary(limit="3") InteropLibrary iop
  ) {
    try {
      return iop.asDate(date).getMonthValue();
    } catch (UnsupportedMessageException ex) {
      CompilerDirectives.transferToInterpreter();
      throw new IllegalStateException(ex);
    }
  }

  @Specialization(guards = "type == 3")
  long executeDay(
      Object state,
      Object self,
      long type,
      Object date,
      @CachedLibrary(limit="3") InteropLibrary iop
  ) {
    try {
      return iop.asDate(date).getDayOfMonth();
    } catch (UnsupportedMessageException ex) {
      CompilerDirectives.transferToInterpreter();
      throw new IllegalStateException(ex);
    }
  }
}
