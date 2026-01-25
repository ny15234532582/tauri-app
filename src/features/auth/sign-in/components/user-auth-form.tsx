import { useState } from 'react'
import { z } from 'zod'
import { useForm } from 'react-hook-form'
import { Link, useNavigate } from '@tanstack/react-router'
import { Loader2, LogIn } from 'lucide-react'
import { toast } from 'sonner'
import { IconFacebook, IconGithub } from '@/assets/brand-icons'
import { useAuthStore } from '@/stores/auth-store'
import { sleep, cn } from '@/lib/utils'
import { Button } from '@/components/ui/button'
import {
  Form,
  FormControl,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from '@/components/ui/form'
import { Input } from '@/components/ui/input'
import { PasswordInput } from '@/components/password-input'
import { invoke } from '@tauri-apps/api/core';

export function UserAuthForm({
  className,
  redirectTo,
  ...props
}) {
  const [isLoading, setIsLoading] = useState(false)
  const navigate = useNavigate()
  const { auth } = useAuthStore()

  // Form标签属性
  const form = useForm({
    defaultValues: {
      email: '',
      password: '',
    },
  })

  // 开始登录
  async function onSubmit(data) {
    setIsLoading(true)

    //开始登录
    const result = await invoke('login', {
      ...data,
    });

    //登录失败，进行消息提示
    if(result.status != 200){
      setIsLoading(false)
      toast.error(result.data);
      return;
    }

    //登录成功，开始跳转
    toast.promise(sleep(2000), {
      loading: '登录成功，开始跳转...',
      success: () => {
        setIsLoading(false)

        // Mock successful authentication with expiry computed at success time
        const mockUser = {
          accountNo: 'ACC001',
          email: data.username,
          role: ['user'],
          exp: Date.now() + 24 * 60 * 60 * 1000, // 24 hours from now
        }

        // Set user and access token
        auth.setUser(mockUser)
        auth.setAccessToken('mock-access-token')

        // Redirect to the stored location or default to dashboard
        const targetPath = redirectTo || '/'
        navigate({ to: targetPath, replace: true })

        return `欢迎登陆, ${data.username}!`
      },
      error: 'Error',
    })
  }

  return (
    <Form {...form}>
      <form
        onSubmit={form.handleSubmit(onSubmit)}
        className={cn('grid gap-3', className)}
        {...props}
      >
        <FormField
          control={form.control}
          name='username'
          render={({ field }) => (
            <FormItem>
              <FormLabel>账号</FormLabel>
              <FormControl>
                <Input placeholder='请输入账号' {...field} />
              </FormControl>
              <FormMessage />
            </FormItem>
          )}
        />
        <FormField
          control={form.control}
          name='password'
          render={({ field }) => (
            <FormItem className='relative'>
              <FormLabel>密码</FormLabel>
              <FormControl>
                <PasswordInput placeholder='请输入密码' {...field} />
              </FormControl>
              <FormMessage />
            </FormItem>
          )}
        />
        <Button className='mt-2' disabled={isLoading}>
          {isLoading ? <Loader2 className='animate-spin' /> : <LogIn />}
          登&nbsp;&nbsp;录
        </Button>
      </form>
    </Form>
  )
}
