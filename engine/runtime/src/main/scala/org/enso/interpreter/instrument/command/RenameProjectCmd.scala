package org.enso.interpreter.instrument.command

import java.util.logging.Level

import org.enso.interpreter.instrument.InstrumentFrame
import org.enso.interpreter.instrument.execution.RuntimeContext
import org.enso.pkg.QualifiedName
import org.enso.polyglot.runtime.Runtime.Api

import scala.collection.mutable
import scala.concurrent.{ExecutionContext, Future}

/** A command that orchestrates renaming of a project name.
  *
  * @param maybeRequestId an option with request id
  * @param request a request for a service
  */
class RenameProjectCmd(
  maybeRequestId: Option[Api.RequestId],
  request: Api.RenameProject
) extends Command(maybeRequestId) {

  /** @inheritdoc */
  override def execute(implicit
    ctx: RuntimeContext,
    ec: ExecutionContext
  ): Future[Unit] =
    Future {
      ctx.locking.acquireWriteCompilationLock()
      try {
        val logger = ctx.executionService.getLogger
        logger.log(
          Level.FINE,
          s"Renaming project [old:${request.namespace}.${request.oldName},new:${request.namespace}.${request.newName}]..."
        )
        val context = ctx.executionService.getContext
        context.renameProject(
          request.namespace,
          request.oldName,
          request.newName
        )
        ctx.contextManager.getAllContexts.values
          .foreach(updateMethodPointers(request.newName, _))
        reply(Api.ProjectRenamed(request.namespace, request.newName))
        logger.log(
          Level.INFO,
          s"Project renamed to ${request.namespace}.${request.newName}"
        )
      } finally {
        ctx.locking.releaseWriteCompilationLock()
      }
    }

  /** Update module name of method pointers in the stack.
    *
    * @param projectName the new project name
    * @param stack the exeution stack
    */
  private def updateMethodPointers(
    projectName: String,
    stack: mutable.Stack[InstrumentFrame]
  ): Unit = {
    stack.mapInPlace {
      case InstrumentFrame(call: Api.StackItem.ExplicitCall, cache, sync) =>
        val moduleName = QualifiedName
          .fromString(call.methodPointer.module)
          .renameProject(projectName)
          .toString
        val methodPointer = call.methodPointer.copy(module = moduleName)
        InstrumentFrame(call.copy(methodPointer = methodPointer), cache, sync)
      case item => item
    }
  }
}
