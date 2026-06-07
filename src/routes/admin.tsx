import { createFileRoute, Outlet, redirect } from '@tanstack/react-router'

export const Route = createFileRoute('/admin')({
  beforeLoad: async ({ context }) => {
    // Auth check
    if (!context.user) {
      throw redirect({ to: '/login', replace: true })
    }
    // Role check
    if (context.user.role !== 'admin') {
      throw redirect({ to: '/', replace: true })
    }
  },
  component: AdminLayout,
})

function AdminLayout() {
  return <Outlet />
}
